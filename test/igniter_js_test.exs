defmodule IgniterJsTest do
  use ExUnit.Case
  doctest IgniterJs

  test "greets the world" do
    assert IgniterJs.hello() == :world
  end
end
