! Advent of Code 2015 Day 01 - Fortran Solution Stub
program day01
    implicit none
    integer :: ios, lu, i, n
    logical :: first
    character(len=16384) :: line

    open(newunit=lu, file="../input.txt", status="old", iostat=ios)
    if (ios /= 0) then
        print *, "Input file not found"
        stop
    end if

    do
        read(lu, '(a)', iostat=ios) line
        if (ios /= 0) exit
    end do
    close(lu)

    n = 0
    first = .false.
    do i = 1, len_trim(line)
        if (line(i:i) == '(') then
            n = n + 1
        else if (line(i:i) == ')') then
            n = n - 1
        end if
        if (.not.first) then
            if (n == -1) then
                print *, 'Santa enters basement at', i
                first = .true.
            end if
        end if
    end do

    print *, 'Santa goes to floor', n

end program day01
