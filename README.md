Learning some Rust by rewriting my Python timecard script.

Script to calculate total hours on a timecard, pasted into stdin. Returns a nice summary of hours by day, and total hours worked.

Example timecard format:

    8/23
    2:30-2:45
    3:45-5:15
    8/24
    5:40-7:20
    8:30-9:40


Example output:

    ---------------------------------------------------
    Paste in timecard data, then <Enter>, then <CTRL-D>
    ---------------------------------------------------
    8/23
    2:30-2:45
    3:45-5:15
    8/24
    5:40-7:20
    8:30-9:40
    ---------------------------------------------------
     8/23: 1.75 hrs
     8/24: 2.83 hrs
    ---------------------------------------------------
    Total: 4.58 hrs
    ---------------------------------------------------



TODO:

- Flag lines with unrecognized format