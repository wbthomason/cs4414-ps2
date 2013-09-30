#include <termios.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <stdio.h>

char parseChar() {

    struct termios oldT, newT;
    char c;

    ioctl(0,TCGETS,&oldT);
    newT=oldT;
    //newT.c_lflag &= ~ECHO; /* echo off */
    newT.c_lflag &= ~ICANON; /*one char @ a time*/
    ioctl(0,TCSETS,&newT); 

    while (1) {
	if(read(0,&c,1)>0)
	    continue;
    }

    ioctl(0,TCSETS,&oldT); /* restore previous terminal mode */
    return c;

}

int main() {
    parseChar();
}
