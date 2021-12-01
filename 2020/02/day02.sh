# day 2 subset 1 solution in shell
input=$(cat pass.in | sed 's/ /_/g')
for line in $input
do
    echo $line
    num=$(echo $line | cut -d_ -f1)
    lo=$(echo $num | cut -d- -f1)
    hi=$(echo $num | cut -d- -f2)
    hi=$(expr $hi + 1)
    letter=$(echo $line | cut -d_ -f2 | sed 's/://g')
    pass=$(echo $line | cut -d_ -f3)

    if test $(echo $pass | egrep -v "($letter.*){$hi}")
    then
        if test $(echo $pass | egrep "($letter.*){$lo}")
        then
            echo $line
        fi
    fi
done