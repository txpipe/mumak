using Cardano.Sync.Data;
using Cardano.Sync.Data.Models;
using Cardano.Sync.Reducers;
using Microsoft.EntityFrameworkCore;

namespace Mumak.Indexer.Data;

public class MumakDbContext
(
    DbContextOptions<MumakDbContext> options,
    IConfiguration configuration
) : CardanoDbContext(options, configuration)
{
    public DbSet<Utxo> Utxos { get; set; }

    override protected void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        modelBuilder.Entity<Utxo>(entity =>
        {
            entity.ToTable("utxos");

            entity.HasKey(e => new { e.Slot, e.Id, e.TxIndex });

            entity.Property(e => e.Id)
                .HasColumnName("id");

            entity.Property(e => e.TxIndex)
                .HasColumnName("tx_index");

            entity.Property(e => e.Slot)
                .HasColumnName("slot");

            entity.Property(e => e.Raw)
                .HasColumnName("raw");
        });
    }
}
