# !/bin/bash
dashes="--- --- --- --- --- --- --- --- ---"

terminalWidth=$(tput cols)
# leftover of T.W minus the dashes length is halved as it is the value applied to one side
terminalCentre=$(( ( $terminalWidth - ${#dashes} ) / 2 ))
padding=$(( $terminalCentre + ${#dashes} ))

PrintDashes(){
    printf "%*s\n" $padding "$dashes"
}

for binFilePath in src/bin/*.rs; do
    binFile=$(basename "$binFilePath" .rs)
    PrintDashes
    echo "Running $binFile...\n"
    cargo run --bin $binFile
    echo "\nFinished $binFile..."
    PrintDashes
done
