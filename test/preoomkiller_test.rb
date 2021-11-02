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
    sh "#{executable} #{command} 2>&1", **args
  end

  let(:executable) { "target/debug/preoomkiller" }
  let(:fake_memory_files){ "-m test/fixtures/max.txt -u test/fixtures/used.txt" }

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

  it "complains when trying to use 100 percent memory" do
    preoomkiller("-p 100 echo 1 2 3", success: false).must_equal(
      "Using >= 100 percent of memory will never happen since the process would already be OOM\n"
    )
  end

  it "runs simple command without waiting" do
    preoomkiller("#{fake_memory_files} echo 1 2 3").must_equal "1 2 3\n"
  end

  it "runs simple command without waiting" do
    time = Benchmark.realtime { preoomkiller("#{fake_memory_files} echo 1 2 3").must_equal "1 2 3\n" }
    time.must_be :<, 0.1
  end

  it "can pass arguments to child" do
    preoomkiller("#{fake_memory_files} echo -n 1 2 3").must_equal "1 2 3"
  end

  it "points to the missing used file when failing" do
    preoomkiller("-i 0 echo 1", success: false).must_equal "Could not open /sys/fs/cgroup/memory/memory.usage_in_bytes\n"
  end

  it "points to the missing max file when failing" do
    preoomkiller("-u test/fixtures/used.txt -i 0 echo 1", success: false).must_equal "Could not open /sys/fs/cgroup/memory/memory.stat\n"
  end

  it "kills child quickly when it is above memory allowance" do
    time = Benchmark.realtime do
      preoomkiller("#{fake_memory_files} -i 0.1 sleep 0.2", success: false).must_equal "Terminated by preoomkiller\n"
    end
    time.must_be :<, 0.2
    time.must_be :>=, 0.1
  end

  it "does not kill child when it has enough memory left" do
    preoomkiller("#{fake_memory_files} -i 0.1 -p 99 sleep 0.2").must_equal ""
  end

  it "kills child when it gets killed itself by SIGTERM" do
    result = nil
    thr = Thread.new { result = preoomkiller("#{fake_memory_files} -i 0.1 -p 99 ./test/fixtures/signal_printer.sh", success: false) }
    sleep 0.1 # let it spin up

    # kill running process
    running = `ps -ef | grep #{executable} | grep -v grep | grep -v 'sh -c'`
    running.count("\n").must_equal 1
    pid = running.split(' ')[1] || raise("No pid found")
    Process.kill(:TERM, Integer(pid))

    thr.join
    result.must_equal "Waiting for signals\nTerminated by forwarded signal\nSIGTERM\n"

    # check if the child is dead
    `ps -ef | grep signal_printer | grep -v grep`.must_equal ""
  end

  it "kills child when it gets killed itself by SIGINT" do
    result = nil
    thr = Thread.new { result = preoomkiller("#{fake_memory_files} -i 0.1 -p 99 ./test/fixtures/signal_printer.sh", success: false) }
    sleep 0.1 # let it spin up

    # kill running process
    running = `ps -ef | grep #{executable} | grep -v grep | grep -v 'sh -c'`
    running.count("\n").must_equal 1
    pid = running.split(' ')[1] || raise("No pid found")
    Process.kill(:INT, Integer(pid))

    thr.join
    result.must_equal "Waiting for signals\nTerminated by forwarded signal\nSIGINT\n"

    # check if the child is dead
    `ps -ef | grep signal_printer | grep -v grep`.must_equal ""
  end
end
