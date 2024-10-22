defmodule Crucible.MyNative do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :crucible,
    crate: "crucible",
    base_url: "https://github.com/TwistingTwists/crucible/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_EXAMPLE_BUILD") in ["1", "true"],
    version: version

  def parse_javascript(_source), do: err()
  def get_comments(_source), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
end
