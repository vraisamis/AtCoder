class LuDecomposition
  # 元の行列
  A = [
	[0.5, 0.5, 0],
	[0, 0.5, 0.5],
	[0.5, 0, 0.5],
    # [2, -3,  1],
    # [1,  1, -1],
    # [3,  5, -7]
  ]

  # 実行
  def exec
    display("A", A)       # 元の行列 A
    lu = lu_decompose(A)  # LU 分解
    display("LU", lu)     # 結果出力
  end

  private

  # LU 分解
  # * U の対角要素を 1 とする
  #
  # @param   a: 元の正方行列 A(n, n)
  # @return  a: LU 分解後の正方行列 A(n, n)
  def lu_decompose(a)
    n = a.size

    begin
      0.upto(n - 1) do |k|
        k.upto(n - 1) do |i|
          sum = 0
          0.upto(k - 1) { |j| sum += a[i][j] * a[j][k] }
          a[i][k] -= sum
        end
        if a[k][k] == 0
          puts "Can't divide by 0 ..."
          exit
        end
        tmp = 1.0 / a[k][k]
        (k + 1).upto(n - 1) do |j|
          sum = 0
          0.upto(k - 1) { |i| sum += a[k][i] * a[i][j] }
          a[k][j] = (a[k][j] - sum) * tmp
        end
      end
      return a
    rescue => e
      raise
    end
  end

  # 行列出力
  #
  # @param s: 行列名
  # @param a: 行列(n * n)
  def display(s, a)
    n = a.size

    begin
      puts "#{s} = "
      n.times do |i|
        n.times { |j| print "  %10.2f" % a[i][j] }
        puts
      end
    rescue => e
      raise
    end
  end
end

LuDecomposition.new.exec
