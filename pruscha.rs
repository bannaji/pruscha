! version = 2.0
! sub i'm     = i am
! sub i'd     = i would
! sub i've    = i have
! sub i'll    = i will
! sub don't   = do not
! sub isn't   = is not
! sub you'd   = you would
! sub you're  = you are
! sub you've  = you have
! sub you'll  = you will
! sub what's  = what is
! sub whats   = what is
! sub what're = what are
! sub what've = what have
! sub what'll = what will
! sub who's   = who is

+ hello
- hello, you can ask me about vit?

+ hi*
@ hello

+ my name is *
- nice to meet you <star>
- nice meeting you <star>

+ thank you
- you are welcome
- welcome
- no problem
- no worries

+ [*] sjt [*] location [*] {weight=10}
- it is located near the lakshmibai block.

+ where is sjt {weight=10}
- @ sjt location

+ [*] scope [*]
- scope is known as scool of computer science and engineering, its head office is loacted on SJT third floor.

+ [*] computer [*]
- The computer science branch school SCOPE is loacted in sjt and its HOD is prof santhi v.

+ [*] location [*]
-location of what

+ sjt
% location of what
- @ sjt location

+ tt
% location of what
- @ tt

+ [*] tt [*]
- tt is located neat the food court.

+[*] hod  [*] {weight=2}
- which school hod you want to know about

+ scope
% which school hod you want to know about
- @ [*] computer [*]

+ site
% which school hod you want to know about
- prof jayakumar

+ [*] sjt [*]
- what do you want to know about sjt?
- try asking "sjt phone number" or "sjt loaction"

+ [*] tt [*]{weight=10}
- what do you want to know about tt?
- try asking "tt phone number" or "tt loaction"

+ where is counselling going on
- anna auditorium

+ [*] anna [*]
- anna auditorium is in front on Main building's second enterance, you can reach there by entering Main building and exiting from other side.

+ [*] counselling schedule [*]
- The counselling schedule has been sent to your mail you can check it there.
 
+ [*] time [*] sjt [*]
- the opening time of sjt is from 7 am to 7:30 pm

+ [*] (chancellor|g visshvanathan) [*]
- our chancellor's office is located in Main building on the ground floor, you will have to take an appointment from MB in order to meet him.

+ [*] counselling [*] {weight= 10}
- what do you want to know about counselling

+ *
- Sorry did not get what you said
- I am afraid that i can not understand you
- I did not get it
- Sorry can you please elaborate that for me
