using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.IO;
using System.ComponentModel;

using CsbBuilder.Audio;
using CsbBuilder.Project;
using CsbBuilder.BuilderNodes;
using CsbBuilder.Serialization;

using SonicAudioLib.IO;
using SonicAudioLib.CriMw.Serialization;
using SonicAudioLib.Archives;

using System.Windows.Forms;

namespace CsbBuilder.Importer
{
    public static class CsbImporter
    {
        public static void Import(string path, CsbProject project)
        {
            var extractor = new DataExtractor
            {
                BufferSize = MainForm.Settings.BufferSize,
                EnableThreading = MainForm.Settings.EnableThreading,
                MaxThreads = MainForm.Settings.MaxThreads
            };

            // Find the CPK first
            string cpkPath = Path.ChangeExtension(path, "cpk");
            bool exists = File.Exists(cpkPath);

            CriCpkArchive cpkArchive = new CriCpkArchive();

            // First, deserialize the main tables
            List<SerializationCueSheetTable> cueSheets = CriTableSerializer.Deserialize<SerializationCueSheetTable>(path, MainForm.Settings.BufferSize);

            /* Deserialize all the tables we need to import.
             * None = 0,
             * Cue = 1,
             * Synth = 2,
             * SoundElement = 4,
             * Aisac = 5,
             * VoiceLimitGroup = 6,
             * VersionInfo = 7,
             */

            List<SerializationCueTable> cueTables = CriTableSerializer.Deserialize<SerializationCueTable>(cueSheets.FirstOrDefault(table => table.TableType == 1).TableData);
            List<SerializationSynthTable> synthTables = CriTableSerializer.Deserialize<SerializationSynthTable>(cueSheets.FirstOrDefault(table => table.TableType == 2).TableData);
            List<SerializationSoundElementTable> soundElementTables = CriTableSerializer.Deserialize<SerializationSoundElementTable>(cueSheets.FirstOrDefault(table => table.TableType == 4).TableData);
            List<SerializationAisacTable> aisacTables = CriTableSerializer.Deserialize<SerializationAisacTable>(cueSheets.FirstOrDefault(table => table.TableType == 5).TableData);
            
            // voice limit groups appeared in the later versions, so check if it exists.
            List<SerializationVoiceLimitGroupTable> voiceLimitGroupTables = new List<SerializationVoiceLimitGroupTable>();

            if (cueSheets.Exists(table => table.TableType == 6))
            {
                voiceLimitGroupTables = CriTableSerializer.Deserialize<SerializationVoiceLimitGroupTable>(cueSheets.FirstOrDefault(table => table.TableType == 6).TableData);
            }

            // Deserialize Sound Element tables

            // BUT BEFORE THAT, see if there's any sound element with Streamed on
            if (soundElementTables.Exists(soundElementTable => soundElementTable.Streaming))
            {
                if (!exists)
                {
                    throw new Exception("Cannot find CPK file for this CSB file. Please ensure that the CPK file is in the directory where the CSB file is, and has the same name as the CSB file, but with .CPK extension.");
                }

                cpkArchive.Load(cpkPath);
            }

            foreach (SerializationSoundElementTable soundElementTable in soundElementTables)
            {
                BuilderSoundElementNode soundElementNode = new BuilderSoundElementNode
                {
                    Name = soundElementTable.Name,
                    ChannelCount = soundElementTable.NumberChannels,
                    SampleRate = soundElementTable.SoundFrequency,
                    Streaming = soundElementTable.Streaming,
                    SampleCount = soundElementTable.NumberSamples
                };

                CriAaxArchive aaxArchive = new CriAaxArchive();

                CriCpkEntry cpkEntry = cpkArchive.GetByPath(soundElementTable.Name);
                if (soundElementNode.Streaming && cpkEntry != null)
                {
                    using (Stream source = File.OpenRead(cpkPath))
                    using (Stream entrySource = cpkEntry.Open(source))
                    {
                        aaxArchive.Read(entrySource);
                    }
                }

                else if (soundElementNode.Streaming && cpkEntry == null)
                {
                    soundElementNode.Intro = soundElementNode.Loop = string.Empty;
                    soundElementNode.SampleRate = soundElementNode.SampleCount = soundElementNode.ChannelCount = 0;
                }

                else
                {
                    aaxArchive.Load(soundElementTable.Data);
                }

                foreach (CriAaxEntry entry in aaxArchive)
                {
                    string outputFileName = Path.Combine(project.AudioDirectory.FullName, soundElementTable.Name.Replace('/', '_'));
                    if (entry.Flag == CriAaxEntryFlag.Intro)
                    {
                        outputFileName += $"_Intro{aaxArchive.GetModeExtension()}";
                        soundElementNode.Intro = Path.GetFileName(outputFileName);
                    }

                    else if (entry.Flag == CriAaxEntryFlag.Loop)
                    {
                        outputFileName += $"_Loop{aaxArchive.GetModeExtension()}";
                        soundElementNode.Loop = Path.GetFileName(outputFileName);
                    }

                    if (soundElementNode.Streaming)
                    {
                        extractor.Add(cpkPath, outputFileName, cpkEntry.Position + entry.Position, entry.Length);
                    }

                    else
                    {
                        extractor.Add(soundElementTable.Data, outputFileName, entry.Position, entry.Length);
                    }
                }

                project.SoundElementNodes.Add(soundElementNode);
            }

            // Deserialize Voice Limit Group tables
            foreach (SerializationVoiceLimitGroupTable voiceLimitGroupTable in voiceLimitGroupTables)
            {
                project.VoiceLimitGroupNodes.Add(new BuilderVoiceLimitGroupNode
                {
                    Name = voiceLimitGroupTable.VoiceLimitGroupName,
                    MaxAmountOfInstances = voiceLimitGroupTable.VoiceLimitGroupNum,
                });
            }

            // Deserialize Aisac tables
            foreach (SerializationAisacTable aisacTable in aisacTables)
            {
                BuilderAisacNode aisacNode = new BuilderAisacNode
                {
                    Name = aisacTable.PathName,
                    AisacName = aisacTable.Name,
                    Type = aisacTable.Type,
                    RandomRange = aisacTable.RandomRange
                };

                // Deserialize the graphs
                List<SerializationAisacGraphTable> graphTables = CriTableSerializer.Deserialize<SerializationAisacGraphTable>(aisacTable.Graph);
                foreach (SerializationAisacGraphTable graphTable in graphTables)
                {
                    BuilderAisacGraphNode graphNode = new BuilderAisacGraphNode
                    {
                        Name = $"Graph{aisacNode.Graphs.Count}",
                        Type = graphTable.Type,
                        MaximumX = graphTable.InMax,
                        MinimumX = graphTable.InMin,
                        MaximumY = graphTable.OutMax,
                        MinimumY = graphTable.OutMin
                    };

                    // Deserialize the points
                    List<SerializationAisacPointTable> pointTables = CriTableSerializer.Deserialize<SerializationAisacPointTable>(graphTable.Points);
                    foreach (SerializationAisacPointTable pointTable in pointTables)
                    {
                        BuilderAisacPointNode pointNode = new BuilderAisacPointNode
                        {
                            Name = $"Point{graphNode.Points.Count}",
                            X = pointTable.In,
                            Y = pointTable.Out
                        };
                        graphNode.Points.Add(pointNode);
                    }

                    aisacNode.Graphs.Add(graphNode);
                }

                project.AisacNodes.Add(aisacNode);
            }

            // Deserialize Synth tables
            foreach (SerializationSynthTable synthTable in synthTables)
            {
                BuilderSynthNode synthNode = new BuilderSynthNode
                {
                    Name = synthTable.SynthName,
                    Type = (BuilderSynthType)synthTable.SynthType,
                    PlaybackType = (BuilderSynthPlaybackType)synthTable.ComplexType,
                    Volume = synthTable.Volume,
                    Pitch = synthTable.Pitch,
                    DelayTime = synthTable.DelayTime,
                    SControl = synthTable.SControl,
                    EgDelay = synthTable.EgDelay,
                    EgAttack = synthTable.EgAttack,
                    EgHold = synthTable.EgHold,
                    EgDecay = synthTable.EgDecay,
                    EgRelease = synthTable.EgRelease,
                    EgSustain = synthTable.EgSustain,
                    FilterType = synthTable.FType,
                    FilterCutoff1 = synthTable.FCof1,
                    FilterCutoff2 = synthTable.FCof2,
                    FilterReso = synthTable.FReso,
                    FilterReleaseOffset = synthTable.FReleaseOffset,
                    DryOName = synthTable.DryOName,
                    Mtxrtr = synthTable.Mtxrtr,
                    Dry0 = synthTable.Dry0,
                    Dry1 = synthTable.Dry1,
                    Dry2 = synthTable.Dry2,
                    Dry3 = synthTable.Dry3,
                    Dry4 = synthTable.Dry4,
                    Dry5 = synthTable.Dry5,
                    Dry6 = synthTable.Dry6,
                    Dry7 = synthTable.Dry7,
                    WetOName = synthTable.WetOName,
                    Wet0 = synthTable.Wet0,
                    Wet1 = synthTable.Wet1,
                    Wet2 = synthTable.Wet2,
                    Wet3 = synthTable.Wet3,
                    Wet4 = synthTable.Wet4,
                    Wet5 = synthTable.Wet5,
                    Wet6 = synthTable.Wet6,
                    Wet7 = synthTable.Wet7,
                    Wcnct0 = synthTable.Wcnct0,
                    Wcnct1 = synthTable.Wcnct1,
                    Wcnct2 = synthTable.Wcnct2,
                    Wcnct3 = synthTable.Wcnct3,
                    Wcnct4 = synthTable.Wcnct4,
                    Wcnct5 = synthTable.Wcnct5,
                    Wcnct6 = synthTable.Wcnct6,
                    Wcnct7 = synthTable.Wcnct7,
                    VoiceLimitType = synthTable.VoiceLimitType,
                    VoiceLimitPriority = synthTable.VoiceLimitPriority,
                    VoiceLimitProhibitionTime = synthTable.VoiceLimitPhTime,
                    VoiceLimitPcdlt = synthTable.VoiceLimitPcdlt,
                    Pan3dVolumeOffset = synthTable.Pan3dVolumeOffset,
                    Pan3dVolumeGain = synthTable.Pan3dVolumeGain,
                    Pan3dAngleOffset = synthTable.Pan3dAngleOffset,
                    Pan3dAngleGain = synthTable.Pan3dAngleGain,
                    Pan3dDistanceOffset = synthTable.Pan3dDistanceOffset,
                    Pan3dDistanceGain = synthTable.Pan3dDistanceGain,
                    Dry0g = synthTable.Dry0g,
                    Dry1g = synthTable.Dry1g,
                    Dry2g = synthTable.Dry2g,
                    Dry3g = synthTable.Dry3g,
                    Dry4g = synthTable.Dry4g,
                    Dry5g = synthTable.Dry5g,
                    Dry6g = synthTable.Dry6g,
                    Dry7g = synthTable.Dry7g,
                    Wet0g = synthTable.Wet0g,
                    Wet1g = synthTable.Wet1g,
                    Wet2g = synthTable.Wet2g,
                    Wet3g = synthTable.Wet3g,
                    Wet4g = synthTable.Wet4g,
                    Wet5g = synthTable.Wet5g,
                    Wet6g = synthTable.Wet6g,
                    Wet7g = synthTable.Wet7g,
                    Filter1Type = synthTable.F1Type,
                    Filter1CutoffOffset = synthTable.F1CofOffset,
                    Filter1CutoffGain = synthTable.F1CofGain,
                    Filter1ResoOffset = synthTable.F1ResoOffset,
                    Filter1ResoGain = synthTable.F1ResoGain,
                    Filter2Type = synthTable.F2Type,
                    Filter2CutoffLowerOffset = synthTable.F2CofLowOffset,
                    Filter2CutoffLowerGain = synthTable.F2CofLowGain,
                    Filter2CutoffHigherOffset = synthTable.F2CofHighOffset,
                    Filter2CutoffHigherGain = synthTable.F2CofHighGain,
                    PlaybackProbability = synthTable.Probability,
                    NLmtChildren = synthTable.NumberLmtChildren,
                    Repeat = synthTable.Repeat,
                    ComboTime = synthTable.ComboTime,
                    ComboLoopBack = synthTable.ComboLoopBack
                };

                project.SynthNodes.Add(synthNode);
            }

            // Convert the cue tables
            foreach (SerializationCueTable cueTable in cueTables)
            {
                BuilderCueNode cueNode = new BuilderCueNode
                {
                    Name = cueTable.Name,
                    Id = cueTable.Id,
                    UserComment = cueTable.UserData,
                    Flags = cueTable.Flags,
                    SynthReference = cueTable.SynthPath
                };
                project.CueNodes.Add(cueNode);
            }

            // Fix links
            for (int i = 0; i < synthTables.Count; i++)
            {
                SerializationSynthTable synthTable = synthTables[i];
                BuilderSynthNode synthNode = project.SynthNodes[i];

                if (synthNode.Type == BuilderSynthType.Single)
                {
                    synthNode.SoundElementReference = synthTable.LinkName;
                }

                // Polyphonic
                else if (synthNode.Type == BuilderSynthType.WithChildren)
                {
                    synthNode.Children = synthTable.LinkName.Split(new char[] { (char)0x0A }, StringSplitOptions.RemoveEmptyEntries).ToList();
                }

                if (!string.IsNullOrEmpty(synthTable.AisacSetName))
                {
                    string[] aisacs = synthTable.AisacSetName.Split(new char[] { (char)0x0A }, StringSplitOptions.RemoveEmptyEntries);
                    string[] name = aisacs[0].Split(new string[] { "::" }, StringSplitOptions.None);
                    synthNode.AisacReference = name[1]; // will add support for multiple aisacs (I'm actually not even sure if csbs support multiple aisacs...)
                }

                if (!string.IsNullOrEmpty(synthTable.VoiceLimitGroupName))
                {
                    synthNode.VoiceLimitGroupReference = synthTable.VoiceLimitGroupName;
                }
            }

            // Extract everything
            extractor.Run();
        }
    }
}
