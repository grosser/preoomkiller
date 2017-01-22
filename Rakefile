require 'bundler/setup'

class Bumper
  FILES = ["Cargo.toml", "src/bin.rs"]

  def initialize(position)
    @position = position
  end

  def bump
    abort "Working directory not clean" unless system("git diff-index --quiet HEAD --")
    new_version = bump_files
    command = "git commit -a -m 'v#{new_version}' && git tag v#{new_version}"
    abort "Failed to bump" unless system(command)
    puts "Comitted v#{new_version}"
  end

  private

  def bump_files
    version = nil
    FILES.each do |f|
      content = File.read(f)
      content.sub!(/"(\d+)\.(\d+)\.(\d+)"/) do
        version = [$1, $2, $3]
        version[@position] = Integer(version[@position]) + 1
        version = version.join(".")
        "\"#{version}\""
      end || abort("Version not found in #{f}")
      File.write(f, content)
    end
    version
  end
end

task :default do
  sh "mtest test"
end

namespace :bump do
  [:major, :minor, :patch].each_with_index do |name, index|
    desc "Bump #{name} version"
    task(name) { Bumper.new(index).bump }
  end
end
