`cpcalendars`
=============

*Really simple CLI app to copy Mac OS Calendar.app calendars sources to other location.*

Copy the calendar resources from the Mac OS Calendar app to the specified directory, building a pseudo-ICBU file. This file constitutes a comprehensive backup of your calendars.

In the first run it will need to be granted permissions for the Calendars app by System Integrity Protection system.


## Motivation

On my Mac OS machine I have some scripts that cannot work as agents because it is prevented by System Integrity Protection, which only allows granting permissions to binary programs. That, coupled with the fact that I wanted to learn the Rust language, motivated me to write this simple program in order to have the conflicting step in a compiled program to authorize and for my scripts routine to work. However, at the end of the day it is a potentially useful generic purpose program.
