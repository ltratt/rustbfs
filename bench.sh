#! /bin/sh

set -e

REPS=30
INTERPS=interp*

if [ ! -f bench.bf ]; then
  curl -o bench.bf https://raw.githubusercontent.com/kostya/benchmarks/master/brainfuck/bench.b
fi

if [ ! -f hanoi.bf ]; then
  curl -o hanoi.bf https://raw.githubusercontent.com/fabianishere/brainfuck/master/examples/hanoi.bf
fi

if [ ! -f manelbrot.bf ]; then
  curl -o mandelbrot.bf https://www.nayuki.io/res/optimizing-brainfuck-compiler/mandelbrot.b.txt
fi

if [ -f results ]; then
  echo "results file already exists: aborting" > /dev/stderr
  exit 1
fi

cargo build --release

batch=`mktemp`
for interp in $INTERPS; do
  for bench in `echo bench.bf hanoi.bf mandelbrot.bf`; do
    echo "-q target/release/${interp} ${bench}" >> $batch
  done
done

multitime -b $batch -n $REPS -s 1 -v 2>&1 | tee results
./multitime_to_html results $REPS

rm $batch
