#!/bin/sh

help() {
    echo "Usage: $0 [options] [command]"
    echo "Options:"
    echo "  -h, --help"
    echo "  -v, --version"
    echo "  -n, --number <number>"
    echo "Commands:"
    echo "  creat"
    echo "  clean"
    echo "Example:"
    echo "  $0 -n 10 creat, creat 10 pairs of can-vxcan"
    echo "  $0 -n 10 clean, clean"
}

creat() {
    sudo modprobe can_raw
    sudo modprobe vxcan
    for i in $(seq 0 $(($number - 1)))
    do
        echo "creat can$i--vxcan$i"
        sudo ip link add can$i type vxcan
        sudo ip link set up can$i
        sudo ip link set up vxcan$i
    done
}

clean() {
    for i in $(seq 0 $(($number - 1)))
    do
        echo "clean can$i--vxcan$i"
        sudo ip link set down can$i
        sudo ip link set down vxcan$i
        sudo ip link del can$i
    done
}

number=1
while [ $# -gt 0 ]
do
    case $1 in
        -h|--help)
            help
            exit 0
            ;;
        -v|--version)
            echo "vxcan.sh 1.0"
            exit 0
            ;;
        -n|--number)
            number=$2
            shift
            ;;
        creat)
            creat
            exit 0
            ;;
        clean)
            clean
            exit 0
            ;;
        *)
            echo "Error: unknown option $1"
            help
            exit 1
            ;;
    esac
    shift
done

help
