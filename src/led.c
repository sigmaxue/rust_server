#include <stdio.h>
#include "softPwm.h"
#include "wiringPi.h"


const int kRedLedPin   = 0;
const int kGreenLedPin = 1;

void InitRedLed( ) {
    softPwmCreate( kRedLedPin, 0, 100 );
    softPwmCreate( kGreenLedPin, 0, 100 );
}

void SetLedColor( char r, char g ) {
    softPwmWrite( kRedLedPin, r );
    softPwmWrite( kGreenLedPin, g );
}


void LedInit() {
    if ( wiringPiSetup( ) == -1 ) {
        printf( "init error" );
    }

    InitRedLed( );

    printf( "%s", __FUNCTION__ );
}


void LedTask() {
    SetLedColor( 0xff, 0x00 );
    delay(500);
    SetLedColor( 0x00, 0xff );
    delay(  500 );
    SetLedColor( 0xff, 0x45 );
    delay(  500 );
    SetLedColor( 0xff, 0xff );
    delay(  500 );
    SetLedColor( 0x7c, 0xfc );
    delay(  500 );
    SetLedColor( 0x00, 0x00 );
    delay(  500 );
}
