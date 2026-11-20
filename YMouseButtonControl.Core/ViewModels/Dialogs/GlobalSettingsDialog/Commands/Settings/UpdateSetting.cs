using System;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using YMouseButtonControl.Infrastructure.Context;

namespace YMouseButtonControl.Core.ViewModels.Dialogs.GlobalSettingsDialog.Commands.Settings;

public static class UpdateSetting<T>
{
    public sealed record Command(string Name, T Value);

    public sealed class Handler(YMouseButtonControlDbContext db)
    {
        public async Task ExecuteAsync(Command c)
        {
            switch (c.Value)
            {
                case string:
                    var entStr = await db.SettingStrings.FirstOrDefaultAsync(x => x.Name == c.Name);
                    if (entStr is null)
                    {
                        entStr = new Domain.Models.SettingString { Name = c.Name, StringValue = c.Value as string };
                        await db.SettingStrings.AddAsync(entStr);
                    }
                    else
                    {
                        entStr.StringValue = c.Value as string;
                    }
                    break;
                case int:
                    var entInt = await db.SettingInts.FirstOrDefaultAsync(x => x.Name == c.Name);
                    if (entInt is null)
                    {
                        entInt = new Domain.Models.SettingInt { Name = c.Name, IntValue = Convert.ToInt32(c.Value) };
                        await db.SettingInts.AddAsync(entInt);
                    }
                    else
                    {
                        entInt.IntValue = Convert.ToInt32(c.Value);
                    }
                    break;
                case bool:
                    var entBool = await db.SettingBools.FirstOrDefaultAsync(x => x.Name == c.Name);
                    if (entBool is null)
                    {
                        entBool = new Domain.Models.SettingBool { Name = c.Name, BoolValue = Convert.ToBoolean(c.Value) };
                        await db.SettingBools.AddAsync(entBool);
                    }
                    else
                    {
                        entBool.BoolValue = Convert.ToBoolean(c.Value);
                    }
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(c.Value));
            }
            await db.SaveChangesAsync();
        }
    }
}
