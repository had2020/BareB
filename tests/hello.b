uint add(a, b, _, _) { // A function can have up to 4 args.
    t0 = a + b;
    return t0;
}

main() {
    param 5;
    param 5;
    t0 = call add;

    param t0;
    call printf;

    t1 = "hello\n";
    param t1;
    call printf;

    return 0;
}
