defmodule CrucibleTest do
  use ExUnit.Case
  doctest Crucible

  test "greets the world" do
    Crucible.parse_javascript("console.log('Hello, World!');")
    |> :json.decode()
    |> dbg()
  end
end
