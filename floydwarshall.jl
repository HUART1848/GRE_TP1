using LinearAlgebra

struct FW
    n::Int64
    m::Int64
    dists::Matrix{Float64}
    preds::Matrix{Float64}
end

function parsedata(path::String)
    data = collect(eachline(path))
    
    n = parse(Int64, data[1])
    m = parse(Int64, data[2])
    dists = fill(Inf, (n, n))
    preds = fill(NaN, (n, n))

    for line in data[3:end]
        linedata = split(line)

        i = parse(Int64, linedata[1])
        j = parse(Int64, linedata[2])
        c = parse(Float64, linedata[3])

        dists[i, j] = c
        preds[i, j] = i
    end

    # Poids nul entre chaque sommet et lui-même
    foreach(e -> dists[e] = 0.0, diagind(dists))

    return FW(n, m, dists, preds)
end

function floydwarshall(fw::FW)
    
end

function main()
    parsedata("data/reseau3.dat")
end
