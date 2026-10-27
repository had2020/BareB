const THING = 6707;

###
These are comments
Also whitespace between tokens is required!
###

returns uint;
param a;
param b;
fn add()
{
    let t0 = a + b;
    return t0;
}

returns void;
fn main()
{
    param 6;
    param THING;
    let t0 = call add;

    if ( t0 == 10 )
    {
        para t0;
        call printf;
    }

    uint[] t1 [5] = "hello";
    param t1;
    call printf;
}
