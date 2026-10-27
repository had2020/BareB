const THING = 670;

###
These are comments
Also whitespace between tokens is required!
###

returns uint;
param uint a;
param uint b;
fn add()
{
    t0 = a + b;
    return t0;
}

returns void;
fn main()
{
    param 6;
    param THING;
    uint t0 = call add;

    if ( t0 == 10 )
    {
        para t0;
        call printf;
    }

    uint[] t1 [5] = ["hello"];
    param t1;
    call printf;
}
