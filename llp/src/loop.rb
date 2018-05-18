def loop(n)
  a = n + 1
  sleep(1)
  loop(a)
end

loop(0)
