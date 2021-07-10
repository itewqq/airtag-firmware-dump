#include <stdio.h>
#include <unistd.h>
#include <time.h>
#include <errno.h>   
#include <wiringPi.h> 

const int POWER_PIN = 0;
const int TRIGGER_PIN = 2;
const int GLITCH_PIN = 3;

const int CMD_ON = (int)'h';
const int CMD_OFF = (int)'l';
const int CMD_DELAY = (int)'d';
const int CMD_PULSE = (int)'p';
const int CMD_GLITCH = (int)'g';

int main() {
    setvbuf(stdin, 0LL, _IONBF, 0LL);
    setvbuf(stdout, 0LL, _IONBF, 0LL);
    setvbuf(stderr, 0LL, _IONBF, 0LL);

    wiringPiSetup();
    pinMode (POWER_PIN, OUTPUT); 
    pinMode (TRIGGER_PIN, INPUT); 
    pinMode (GLITCH_PIN, OUTPUT); 
    unsigned int delay, pulse;

    for(;;) { 
        unsigned int cmd = getchar();
        switch(cmd){
            case CMD_ON: 
                digitalWrite(POWER_PIN, HIGH); 
                break;
            case CMD_OFF: 
                digitalWrite(POWER_PIN, LOW);
                break;
            case CMD_DELAY: 
                fread(&delay,1,4,stdin);
                break;
            case CMD_PULSE:
                fread(&pulse,1,4,stdin);
                break;
            case CMD_GLITCH:
                digitalWrite(POWER_PIN, HIGH);
                while(digitalRead(TRIGGER_PIN) == LOW);
                for(unsigned int i=0;i<delay;++i) {
                    __asm__ __volatile__("nop");
                }
                digitalWrite(GLITCH_PIN, HIGH);
                for(unsigned int  i=0;i<pulse;++i) {
                    __asm__ __volatile__("nop");
                }
                digitalWrite(GLITCH_PIN, LOW); 
        }
        fwrite("Y",1,1,stdout);
    } 
}