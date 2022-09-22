# we want to randomize which process is actually the "noisy" one, so this makes sure each run won't just be the same (boring!)
shuf .env.suspects > .env.suspects.tmp && mv .env.suspects.tmp .env.suspects
for number in 1 2 3 4 5; do
  sed -i "${number} s/./LOG_LEVEL=&/" .env.suspects
done < .env.suspects
