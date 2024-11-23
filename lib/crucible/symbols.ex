defmodule Crucible.Symbols do
  defstruct [:resource]

  @type t() :: %__MODULE__{resource: reference()}

  @spec ast_for_symbols_from_buffer(String.t(), String.t()) :: {:ok, t()} | {:error, term()}
  defdelegate ast_for_symbols_from_buffer(source_code_string, filename),
    to: Crucible.Native,
    as: :ast_for_symbols_from_buffer

  @spec names(t()) :: term()
  defdelegate names(module), to: Crucible.Native, as: :symbol_names
end
