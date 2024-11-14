defmodule Crucible do
  use Rustler, otp_app: :crucible, crate: "crucible"

  # def parse_javascript(_source), do: err()
  # def get_comments(_source), do: err()

  # imports
  def ast_for_imports_from_buffer(_source_code, _filename), do: err()
  def local_name_exists(_module, _import_name), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

end
