# hhah this doesnt really work but idk just look at the python file
input=$(cat input.txt | sed 's/ /_/g')
for line in $input
do
    #echo $line
    num=$(echo $line | cut -d_ -f1)
    lo=$(echo $num | cut -d- -f1)
    hi=$(echo $num | cut -d- -f2)
    #echo $hi
    letter=$(echo $line | cut -d_ -f2 | sed 's/://g')
    pass=$(echo $line | cut -d_ -f3)

    if test $(echo $pass | cut -c$lo | egrep "$letter") && test ! $(echo $pass | cut -c$hi | egrep "$letter")
    then
        echo ">> $line"
    elif test $(echo $pass | cut -c$lo | egrep "$letter") && test ! $(echo $pass | cut -c$hi | egrep "$letter")
    then
        echo ">> $line"
    fi

done