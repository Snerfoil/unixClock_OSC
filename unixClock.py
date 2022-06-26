from pythonosc import udp_client #requires python-osc to be installed
import time

def angleTime(a,b):
    
    

    return (time.time()/(2**a))%(2**(b+1))/(2**(b+1))
    #a: start byte index
    #b: end byte index
    #returns a value from 0 to 1 indicating the value within that set of bytes.

def angleTime2(a,b): #same but using bitshifts and pure integer math
    return (((time.time_ns()//(10**9))>>a)%(1<<(b+1)))/(1<<(b+1))
    
def angleTime3(a,b):
    return (((time.time_ns()//(10**9))>>a)%b)/b
    #a: start order
    #b: orderLength
    #I did my math wrong, rather I didnt set the variables to the correct lengths

def ringAngles():
    return [
        angleTime( 0, 5),#sec
        angleTime( 6,11),#min
        angleTime(12,15),#hr
        angleTime(16,18),#day
        angleTime(19,20),#week
        angleTime(21,24),#month
        angleTime(25,27),#year
        angleTime(28,31),#decade
        angleTime(32,34),#century
        angleTime( 0,63),#unix
    ]
    
def ringAngles2():
    return [
        angleTime3( 0,64),#sec
        angleTime3( 6,64),#min
        angleTime3(12,16),#hr
        angleTime3(16, 8),#day
        angleTime3(19, 4),#week
        angleTime3(21,16),#month
        angleTime3(25, 8),#year
        angleTime3(28,16),#decade
        angleTime3(32, 8),#century
        angleTime3( 0,9223372036854775807),#unix
    ]

paramNames = [
    "ringSec",
    "ringMin",
    "ringHr",
    "ringDay",
    "ringWeek",
    "ringMonth",
    "ringYear",
    "ringDecade",
    "ringCentury",
    "ringUnix"
] #this could end up using 72 bytes in VRC's thing.
oscVRC = udp_client.SimpleUDPClient('127.0.0.1',9000)

#oscVRC.send_message("/avatar/parameters/animatorParamName",0.0)

while(True):
    time.sleep(1.0) # so the script doesnt abuse VRC too much.
    temp = ringAngles2()
    for i in range(0,10):
        oscVRC.send_message("/avatar/parameters/"+paramNames[i],temp[i])

#unknown if this works.
'''
the jist is that the rings on arceus's back are meant to be animation with this thing. basically each ring spins around 360deg in this order

min ->
sec <-
hr  ->

week  ->
day   <-
month ->

decade  <-
year    ->
century <-

unix <-

(this is looking at the rings from the back top is outer most, bottom is that inner yin-yag symbol, the bones are also named the same).

each ring just needs two keyframes, linearly interpolated doing a 360deg spin in said direction. controled by the respective animator expression, parameter... thingy you know what I mean.
'''