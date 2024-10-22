defmodule IgniterJs do
  @moduledoc """
  Documentation for `IgniterJs`.
  """

  def test_example do
    Crucible.parse_javascript("console.log('Hello, World!');")
    |> :json.decode()
  end
end
