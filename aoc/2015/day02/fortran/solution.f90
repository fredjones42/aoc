! Advent of Code 2015 Day 02 - Fortran Solution Stub
program day02
    implicit none
    integer :: ios, lu
    integer :: l, w, h, lw, lh, wh, mn, vol
    integer :: area, length
    character(len=16384) :: line

    open(newunit=lu, file="../input.txt", status="old", iostat=ios)
    if (ios /= 0) then
        print *, "Input file not found"
        stop
    end if

    area = 0
    length = 0

    do
        read(lu, '(a)', iostat=ios) line
        if (ios /= 0) exit
        call parse_line(line, l, w, h)
        lw = l * w
        lh = l * h
        wh = w * h
        mn = min(lw, lh, wh)
        area = area + 2 * lw + 2 * lh + 2 * wh + mn
        lw = 2*l + 2*w
        lh = 2*l + 2*h
        wh = 2*w + 2*h
        mn = min(lw, lh, wh)
        vol = l * w * h
        length = length + mn + vol
    end do

    close(lu)

    print *, 'The elves should order', area, 'square feet of wrapping paper'
    print *, 'The elves should order', length, 'feet of ribbon'

contains

    subroutine parse_line(line, l, w, h)

        character(*), intent(in) :: line
        integer, intent(out) :: l, w, h

        integer :: i, ios, j, jo, n
        character(256) :: fmt

        l = 0
        w = 0
        h = 0

        i = index(line, 'x')
        if (i == 0) stop 'cannot find first x'

        write(fmt, '(2x,i10.10)') i - 1
        n = len_trim(fmt) + 1
        fmt(1:1) = '('
        fmt(2:2) = 'i'
        fmt(n:n) = ')'

        read(line(1:i-1), fmt, iostat=ios) l
        if (ios /= 0) stop 'cannot parse first number'

        jo = index(line(i+1:), 'x')
        if (jo == 0) stop 'cannot find second x'
        j = i + jo
        if (line(j:j) /= 'x') stop 'programming error'

        write(fmt, '(2x,i10.10)') j - i - 1
        n = len_trim(fmt) + 1
        fmt(1:1) = '('
        fmt(2:2) = 'i'
        fmt(n:n) = ')'

        read(line(i+1:j-1), fmt, iostat=ios) w
        if (ios /= 0) stop 'cannot parse second number'

        write(fmt, '(2x,i10.10)') len_trim(line) - j
        n = len_trim(fmt) + 1
        fmt(1:1) = '('
        fmt(2:2) = 'i'
        fmt(n:n) = ')'

        read(line(j+1:len_trim(line)), fmt, iostat=ios) h
        if (ios /= 0) stop 'cannot parse third number'
    end subroutine parse_line

end program day02
