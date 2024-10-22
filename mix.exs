defmodule Crucible.MixProject do
  use Mix.Project

  @version "0.1.1"
  @source_url "https://github.com/TwistingTwists/crucible"

  def project do
    [
      app: :crucible,
      version: @version,
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      package: package(),
      deps: deps()
    ]
  end

  def package() do
    [
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => @source_url},
      maintainers: ["Abhishek Tripathi"]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, "~> 0.8"},
      # only when you need forced compilation
      {:rustler, ">= 0.0.0", optional: true}
    ]
  end
end
