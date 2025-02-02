defmodule BestTimeToBuyAndSellStock do
  @spec max_profit(prices :: [integer]) :: integer
  def max_profit(prices) do
    calculate(prices, 0, nil)
  end

  defp calculate([], profit, _min_price), do: profit

  defp calculate([price | rest], profit, nil) do
    calculate(rest, profit, price)
  end

  defp calculate([price | rest], profit, min_price) do
    new_profit = max(profit, price - min_price)
    new_min_price = min(min_price, price)
    calculate(rest, new_profit, new_min_price)
  end
end
