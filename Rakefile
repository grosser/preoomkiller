require 'bundler/setup'

class Bumper
  FILES = ["Cargo.toml", "src/bin.rs", "Readme.md"]

  def initialize(position)
    @position = position
  end

  def bump
    abort "Working directory not clean" unless system("git diff-index --quiet HEAD --")
    new_version = bump_files
    update_usage_in_readme
    commit new_version
  end

  private

  def bump_files
    File.read(FILES.first) =~ /"(\d+)\.(\d+)\.(\d+)"/ || raise("Version not found in #{FILES.first}")
    old_version = [$1, $2, $3]
    new_version = old_version.dup
    new_version[@position] = Integer(old_version[@position]) + 1
    new_version = new_version.join(".")
    old_version = old_version.join(".")

    FILES.each do |file|
      content = File.read(file).sub!(old_version, new_version) || abort("Version not found in #{file}")
      File.write(file, content)
    end

    new_version
  end

  def update_usage_in_readme
    file = "Readme.md"
    marker = "<!-- Updated by rake bump:patch -->\n"
    marker_rex = /#{Regexp.escape(marker)}.*#{Regexp.escape(marker)}/m
    usage = `cargo run -- -h`
    usage_with_marker = "#{marker}```\n#{usage}```\n#{marker}"
    raise "Unable to get usage" unless $?.success?
    File.write(file, File.read(file).sub!(marker_rex, usage_with_marker) || raise("Unable to find #{marker.strip} in #{file}"))
  end

  def commit(new_version)
    command = "git commit -a -m 'v#{new_version}'"
    abort "Failed to bump" unless system(command)
    puts "Comitted v#{new_version}"
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
