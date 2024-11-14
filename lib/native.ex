defmodule Crucible.Native do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :crucible,
    crate: "crucible",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_EXAMPLE_BUILD") in ["1", "true"],
    version: version

  # imports
  def ast_for_imports_from_buffer(_source_code, _filename), do: err()
  def local_name_exists(_module, _import_name), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
end
