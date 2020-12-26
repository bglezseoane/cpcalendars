`cpcalendars`
=============

*Really simple CLI app to copy Mac OS Calendar.app calendars sources to other location.*

![Status](http://www.borja.glezseoane.es/img/project-status-label-development.svg "Status: development")

Copy the calendar resources from the Mac OS Calendar app to the specified directory, building a pseudo-ICBU file. This file constitutes a comprehensive backup of your calendars.

In the first run it will need to be granted permissions for the Calendars app by System Integrity Protection system.



## Install

The recommended installation option is use Homebrew with the command:

```sh
brew install bglezseoane/tap/cpcalendars
```

You can only install the program with Cargo, with the command:

```sh
cargo install cpcalendars
```



## Use

The normal use of the tool is very simple. You only need to run:

```sh
cpcalendars <destination>
```

A folder named `All Calendars.icbu` will saved into `<destination>`.


### Achieve permissions against Mac OS System Integrity Protection

If your current **terminal app** has **Calendars.app access** or **full disk access** and you use `cpcalendars` **since it**, the tool will works fine. If you only want to work with `cpcalendars` **since your terminal** or since scripts used by you since your terminal, **the above is sufficient** and the following steps are irrelevant to your use case.

Otherwise, if you want to use the tool since an script routine as **`launchd` agent**, the tool is going to fail due to System Integrity Protection.

Mac OS System Integrity Protection block `cpcalendars` because the tool try to access the protected Calendars.app directories and it doesn't inherit permissions —**using it since the terminal, inherit terminal granted ones**—. To achieve permissions, you need to run the following command:

```sh
open -a '/usr/local/bin/cpcalendars' --args "<tmp_destination>"
```

When run the above command, **a pop up window appear** and you can grant access to your calendars to `cpcalendars`. **Do it** and the next time the tool will work.

The path `/usr/local/bin/cpcalendars` could be different if you have installed with Cargo. Any case, you can check it with `which cpcalendars`.

If your agent only needs to run the tool and no more stuff, you can add the following lines to the agent `Info.plist` and it will work (after previously indicated first execution):

```xml
<key>ProgramArguments</key>
    <array>
      	<string>/usr/local/bin/cpcalendars</string>
      	<string>destination</string>
    </array>
```

For any reason, to work with the tool since the `launchd` agent and **integrated in a shell script**, you need to always run it as `open -a '/usr/local/bin/cpcalendars' --args "<destination>"` and not directly as a command (e.g. `/usr/local/bin/cpcalendars <destination>`). I.e., if you use, e.g., the following agent specification...

```xml
<key>ProgramArguments</key>
    <array>
      	<string>/Users/You/scripts/script_which_calls_cpcalendars.sh</string>
    </array>
```

... you need to run `cpcalendars` as follows:

```sh
# 'script_which_calls_cpcalendars.sh'
# ...

open -a '/usr/local/bin/cpcalendars' --args "<destination>"
# And not '/usr/local/bin/cpcalendars <destination>

# ...
```

When run with `open -a`, the command don't return an error code if the launched application fails, so in order to integrate this step in a well designed script, the next approach is recommended:

```sh
# ...

# Create temporary file to save the output of the 'open' command
TMP_OPEN_CPCALENDARS_STDOUT=$(mktemp -t open_cpcalendars)

# Run and wait
open -W -a '/usr/local/bin/cpcalendars' \
	--stdout "$TMP_OPEN_CPCALENDARS_STDOUT" \
	--stderr "$TMP_OPEN_CPCALENDARS_STDOUT" \
	--args <destination>

# Check and act attending to the process success
if grep -Fxq 'Error' "$TMP_OPEN_CPCALENDARS_STDOUT" \
	|| ! grep -Fxq 'Process finished successfully.' "$TMP_OPEN_CPCALENDARS_STDOUT"
then
	>&2 echo "Error using 'cpcalendars'."
	rm "$TMP_OPEN_CPCALENDARS_STDOUT"
	exit 1 
fi

# Clean
rm "$TMP_OPEN_CPCALENDARS_STDOUT"

# ...
```



## Motivation

On my Mac OS machine I have some scripts that cannot work as agents because it is prevented by System Integrity Protection, which only allows granting permissions to binary programs. That, coupled with the fact that I wanted to learn the Rust language, motivated me to write this simple program in order to have the conflicting step in a compiled program to authorize it and can maintain my script routine working elegantly —and not pseudo-compiling shell scripts inserted them in a compiled language program—. However, at the end of the day `cpcalendars` is a potentially useful generic purpose program.
