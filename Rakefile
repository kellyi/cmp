task default: %w[build]

desc "Build Docker container"
task :build do
  puts "Building Docker containers from docker-compose.yml ->"
  sh "docker-compose build"
end

desc "Run llp container"
task :llp do
  puts "Running llp container bash ->"
  sh "docker-compose run llp"
end
