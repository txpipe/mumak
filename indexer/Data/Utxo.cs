using Cardano.Sync.Data.Models;

namespace Mumak.Indexer.Data;

public record Utxo(
    string Id,
    ulong TxIndex,
    ulong Slot,
    byte[] Raw
) : IReducerModel;