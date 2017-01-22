require 'bundler/setup'
require 'maxitest/autorun'

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
end
