# defmodule Crucible.Native do
#   mix_config = Mix.Project.config()
#   version = mix_config[:version]
#   github_url = mix_config[:package][:links]["GitHub"]

#   # if Mix.env() == :dev do
#     use Rustler, otp_app: :crucible, crate: "crucible"
#   # else
#     # use RustlerPrecompiled,
#     #   otp_app: :crucible,
#     #   crate: "crucible",
#     #   base_url: "https://github.com/TwistingTwists/crucible/releases/download/v#{version}",
#     #   force_build: System.get_env("RUSTLER_PRECOMPILATION_EXAMPLE_BUILD") in ["1", "true"],
#     #   version: version
#   # end

#   # use Rustler, otp_app: :crucible, crate: "crucible"

#   def parse_javascript(_source), do: err()
#   def get_comments(_source), do: err()

#   defp err(), do: :erlang.nif_error(:nif_not_loaded)
# end
