# we want to randomize which process is actually the "noisy" one, so this makes sure each run won't just be the same (boring!)
shuf .env.suspects > .env.suspects.tmp && mv .env.suspects.tmp .env.suspects
count=1
for LANGUAGE in NODE PYTHON GOLANG; do
  sed -i "${count} s/./${LANGUAGE}_LOG=&/" .env.suspects
  (( count++ ))
done < .env.suspects
