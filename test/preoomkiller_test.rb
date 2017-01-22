require 'bundler/setup'
require 'maxitest/autorun'
require 'benchmark'

# build executable we will test
`cargo build`

describe 'Preoomkiller' do
  def sh(command, success: true)
    result = `#{command}`
    raise "FAILED\n#{command}\n#{result}" if $?.success? != success
    result
  end

  def preoomkiller(command, **args)
    sh "target/debug/preoomkiller #{command}", **args
  end

  it "shows usage when run without arguments" do
    result = preoomkiller "", success: false
    result.must_include "Usage:"
  end

  ["-h", "--help"].each do |option|
    it "shows usage when run with #{option}" do
      result = preoomkiller option
      result.must_include "Usage:"
    end
  end

  ["-v", "--version"].each do |option|
    it "shows version when run with #{option}" do
      result = preoomkiller option
      result.must_match /\A\d+\.\d+\.\d+\n\z/
    end
  end

  it "runs simple command without waiting" do
    preoomkiller("echo 1 2 3").must_equal "1 2 3\n"
  end

  it "runs simple command without waiting" do
    time = Benchmark.realtime { preoomkiller("echo 1 2 3").must_equal "1 2 3\n" }
    time.must_be :<, 0.1
  end

  it "can pass arguments to child" do
    preoomkiller("echo -n 1 2 3").must_equal "1 2 3"
  end
end
