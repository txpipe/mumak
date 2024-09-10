using System.Linq.Expressions;
using Cardano.Sync.Extensions;
using Cardano.Sync.Reducers;
using Microsoft.EntityFrameworkCore;
using Mumak.Indexer.Data;
using PallasDotnet.Models;

namespace Mumak.Indexer.Reducers;

public class UtxoReducer(IDbContextFactory<MumakDbContext> dbContextFactory) : IReducer<Utxo>
{
    public async Task RollBackwardAsync(NextResponse response)
    {
        using MumakDbContext dbContext = await dbContextFactory.CreateDbContextAsync();
        dbContext.Utxos.RemoveRange(dbContext.Utxos.AsNoTracking().Where(b => b.Slot > response.Block.Slot));
        await dbContext.SaveChangesAsync();
        dbContext.Dispose();
    }

    public async Task RollForwardAsync(NextResponse response)
    {
        using MumakDbContext dbContext = await dbContextFactory.CreateDbContextAsync();
        
        Expression<Func<Utxo, bool>> predicate = PredicateBuilder.False<Utxo>();
        
        var inputs = response.Block.TransactionBodies
            .SelectMany(tx => tx.Inputs)
            .Select(input => new { input.Id, input.Index })
            .ToList();

        inputs.ForEach(input =>
        {
            predicate = predicate.Or(u => u.Id == input.Id.ToHex() && u.TxIndex == input.Index);
        });

        dbContext.Utxos.RemoveRange(dbContext.Utxos.AsNoTracking().Where(predicate));

        response.Block.TransactionBodies.ToList()
        .ForEach(tx => tx.Outputs.
            ToList()
            .ForEach(output =>
            {
                dbContext.Utxos.Add(new Utxo(
                    tx.Id.ToHex(),
                    output.Index,
                    response.Block.Slot,
                    output.Raw
                ));
            })
        );

        await dbContext.SaveChangesAsync();
    }
}