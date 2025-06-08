using HealthCheckApp.Models;
using Microsoft.AspNetCore.Hosting;
using System;
using System.IO;
using System.Text.Json;

namespace HealthCheckApp.Services
{
    public class ConfigurationService
    {
        public HealthCheckConfig Config { get; private set; }

        public ConfigurationService(IWebHostEnvironment env)
        {
            try
            {
                var configFilePath = Path.Combine(env.ContentRootPath, "healthcheckconfig.json");
                if (File.Exists(configFilePath))
                {
                    var jsonContent = File.ReadAllText(configFilePath);
                    Config = JsonSerializer.Deserialize<HealthCheckConfig>(jsonContent, new JsonSerializerOptions
                    {
                        PropertyNameCaseInsensitive = true,
                        AllowTrailingCommas = true,
                        ReadCommentHandling = JsonCommentHandling.Skip
                    });
                }
                else
                {
                    // Log error: Configuration file not found
                    Console.WriteLine($"Error: Configuration file '{configFilePath}' not found.");
                    Config = new HealthCheckConfig(); // Initialize with empty config
                }
            }
            catch (Exception ex)
            {
                // Log error: Failed to load or parse configuration
                Console.WriteLine($"Error loading or parsing configuration: {ex.Message}");
                Config = new HealthCheckConfig(); // Initialize with empty config on error
            }

            // Ensure Config is not null even if everything fails
            Config ??= new HealthCheckConfig();
        }
    }
}
