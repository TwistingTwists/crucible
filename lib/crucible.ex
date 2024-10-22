defmodule Crucible do
  use Rustler, otp_app: :crucible, crate: "crucible"

  def parse_javascript(_source), do: :erlang.nif_error(:nif_not_loaded)
  def get_comments(_source), do: :erlang.nif_error(:nif_not_loaded)
end
