! Advent of Code 2019 Day 02 - Fortran Solution Stub
program day02
    implicit none
    integer :: ios, lu
    character(len=16384) :: line

    open(newunit=lu, file="../input.txt", status="old", iostat=ios)
    if (ios /= 0) then
        print *, "Input file not found"
        stop
    end if

    do
        read(lu, '(a)', iostat=ios) line
        if (ios /= 0) exit
        print *, trim(line)
    end do
    close(lu)

    print *, "No Fortran solution has been written yet"
end program day02
