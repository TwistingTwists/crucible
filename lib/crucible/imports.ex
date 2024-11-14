defmodule Crucible.Imports do
  defstruct [:resource]

  @type t() :: %__MODULE__{resource: reference()}

  @spec ast_for_imports_from_buffer(String.t(), String.t()) :: {:ok, t()} | {:error, term()}
  defdelegate ast_for_imports_from_buffer(source_code_string, filename),
    to: Crucible.Native,
    as: :ast_for_imports_from_buffer

  @spec local_name_exists(t(), String.t()) :: boolean()
  defdelegate local_name_exists(module, import_name), to: Crucible.Native, as: :local_name_exists
end
