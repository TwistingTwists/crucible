defmodule Crucible do
  use Rustler, otp_app: :igniter_js, crate: "crucible"

  def parse_javascript(_source), do: :erlang.nif_error(:nif_not_loaded)
  def get_comments(_source), do: :erlang.nif_error(:nif_not_loaded)
end
