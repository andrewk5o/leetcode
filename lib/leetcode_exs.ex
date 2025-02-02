defmodule LeetcodeExs do
  @moduledoc """
  Documentation for `LeetcodeExs`.
  """
  @spec main() :: :ok
  def main do
    IO.puts("Leetcode exercises")
    IO.puts("Best time to buy and sell stock")
    IO.puts(BestTimeToBuyAndSellStock.max_profit([7, 1, 5, 3, 6, 4]))
    IO.puts("Best time to buy and sell stock 2")
    IO.puts(BestTimeToBuyAndSellStockII.max_profit([7, 1, 5, 3, 6, 4]))
  end
end
