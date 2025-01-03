! Advent of Code 2015 Day 03 - Fortran Solution Stub
program day03
    implicit none
    integer :: i, ios, ir, j, jr, k, lu, n, sol1, sol2
    integer, allocatable :: grid(:, :)
    character(len=16384) :: line

    open(newunit=lu, file="../input.txt", status="old", iostat=ios)
    if (ios /= 0) then
        print *, "Input file not found"
        stop
    end if

    do
        read(lu, '(a)', iostat=ios) line
        if (ios /= 0) exit
        n = len_trim(line)
    end do
    close(lu)

    allocate(grid(-n:n, -n:n))
    grid = 0
    i = 0
    j = 0
    grid(i, j) = 1
    do k = 1, n
        if (line(k:k) == '^') then
            j = j + 1
        else if (line(k:k) == 'v') then
            j = j - 1
        else if (line(k:k) == '>') then
            i = i + 1
        else if (line(k:k) == '<') then
            i = i - 1
        end if
        grid(i, j) = grid(i, j) + 1
    end do
    sol1 = 0
    do j = -n, n
        do i = -n, n
            if (grid(i, j) > 0) sol1 = sol1 + 1
        end do
    end do
    grid = 0
    i = 0
    j = 0
    ir = 0
    jr = 0
    grid(i, j) = 2
    do k = 1, n
        if (mod(k, 2) == 1) then
            if (line(k:k) == '^') then
                j = j + 1
            else if (line(k:k) == 'v') then
                j = j - 1
            else if (line(k:k) == '>') then
                i = i + 1
            else if (line(k:k) == '<') then
                i = i - 1
            end if
            grid(i, j) = grid(i, j) + 1
        else
            if (line(k:k) == '^') then
                jr = jr + 1
            else if (line(k:k) == 'v') then
                jr = jr - 1
            else if (line(k:k) == '>') then
                ir = ir + 1
            else if (line(k:k) == '<') then
                ir = ir - 1
            end if
            grid(ir, jr) = grid(ir, jr) + 1
        end if
    end do
    sol2 = 0
    do j = -n, n
        do i = -n, n
            if (grid(i, j) > 0) sol2 = sol2 + 1
        end do
    end do
    deallocate(grid)

    print *, sol1, "houses received at least one present."
    print *, "With Robo-Santa..."
    print *, sol2, "houses received at least one present."
end program day03
