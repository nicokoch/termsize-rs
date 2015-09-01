/*
* This file is used to expose the ioctl
* number of TIOCGWINSZ on unix-like
* systems. The use of this program is
* necessary, since it is not guaranteed
* that ioctl numbers are constant across
* platforms. As the kernel exposes this
* number as a macro, we have to simply
* redefine it as a constant so we can
* bind to it from rust.
*/
#include <linux/termios.h>
const int tiocgwinsz = TIOCGWINSZ;
