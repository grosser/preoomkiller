require 'bundler/setup'

class Bumper
  FILES = ["Cargo.toml", "src/bin.rs", "Readme.md"]
  TARGETS = [
    'x86_64-apple-darwin',
    # 'i686-unknown-linux-gnu' # TODO: fails :/
  ]
  VERSION_REX = /"((\d+)\.(\d+)\.(\d+))"/

  def initialize(position)
    @position = position
  end

  def bump
    # abort "Working directory not clean" unless system("git diff-index --quiet HEAD --")
    # new_version = bump_files
    # update_usage_in_readme
    # commit new_version
    compile
  end

  private

  def sh(command, success: true)
    puts command
    result = `#{command}`
    raise "FAILED\n#{command}\n#{result}" if $?.success? != success
    result
  end

  def bump_files
    File.read(FILES.first) =~ VERSION_REX || raise("Version not found in #{FILES.first}")
    old_version = [$2, $3, $4]
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
    usage = sh "cargo run -- -h"
    usage_with_marker = "#{marker}```\n#{usage}```\n#{marker}"
    File.write(file, File.read(file).sub!(marker_rex, usage_with_marker) || raise("Unable to find #{marker.strip} in #{file}"))
  end

  def commit(new_version)
    sh  "git commit -a -m 'v#{new_version}'"
    puts "Comitted v#{new_version}"
  end

  # if this fails try rustup target add #{tripple}
  def compile
    TARGETS.each do |tripple|
      sh  "xargo build --release --target #{tripple}"
    end
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

desc "Release new version"
task :release do
  version = File.read('Cargo.toml')[Bumper::VERSION_REX, 1]
  version = "v#{version}"
  sh "git tag #{version}"
  begin
    sh "cargo publish"
  rescue
    sh "git tag -d #{version}"
  else
    sh "git push --tags"
  end
end
