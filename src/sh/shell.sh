#!/bin/sh

i=0
p=2
while [ "$i" -lt 1000 ] # while (i < 1000)
do
  j=2
  f=0
  while [ "$j" -lt "$p" ]
  do
    if [ `expr $p % $j` -eq 0 ]
    then
      f=`expr $f + 1`
    fi
    j=`expr $j + 1` # j++
  done
  if [ "$f" -eq 0 ]
  then
    echo "$p"
    j=0 # reset the loop
    i=`expr $i + 1`
  fi
  p=`expr $p + 1`
done
