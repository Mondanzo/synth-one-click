import 'dart:typed_data';
import 'package:http/http.dart' as http;
import 'package:win32_registry/win32_registry.dart';
import 'dart:io';
import 'package:path/path.dart' as p;

const urls = {
  "beatmaps": "https://synthriderz.com/api/beatmaps/{{id}}/download",
  "beatmaps-hash": "https://synthriderz.com/api/beatmaps/hash/download/{{id}}",
  "stages":
      "https://synthriderz.com/api/models/stages/{{id}}/download?file_id={{id2}}",
  "avatars": "https://synthriderz.com/api/models/avatars/{{id}}/download"
};

void registerInstaller() {
  var key = Registry.currentUser.createKey("SOFTWARE\\Classes\\synthriderz");
  key.createValue(RegistryValue("URL Protocol", RegistryValueType.string, ""));
  var shell = key.createKey("shell\\open\\command");
  shell.createValue(RegistryValue(
      "", RegistryValueType.string, "\"${getPath()}\" \"--install\" \"%1\""));
  shell.close();
  key.close();
}

Future<bool> checkIfSR(String path) async {
  var folder = Directory(path);
  if (!await folder.exists()) return false;
  if (!await Directory(p.join(path, "CustomSongs")).exists()) return false;
  if (!await Directory(p.join(path, "CustomStages")).exists()) return false;
  if (!await Directory(p.join(path, "Avatars")).exists()) return false;
  return true;
}

String? getSynthRidersFolder() {
  var path = Registry.currentUser.getValue(
      "com.synthriders.installpath_h4259148619",
      path: "SOFTWARE\\Kluge Interactive\\SynthRiders");

  if (path != null && path.type == RegistryValueType.binary) {
    return String.fromCharCodes(path.data as Uint8List);
  }
}

String getPath() {
  return Platform.executable;
}

late http.Client taskClient;

Future<void> downloadFile(String uri, String folder) async {
  try {
    print("Trying to download: ${uri.toString()}");
    var response = await taskClient.get(Uri.parse(uri));
    var fileHeader = response.headers["content-disposition"];
    if (fileHeader == null) {
      print("Couldn't get fileheader! Error!");
      return;
    }
    var filename = RegExp(r'filename="(?<filename>.+?)"')
        .firstMatch(fileHeader)
        ?.namedGroup("filename");
    print("Downloading file ${filename}...");
    var file = File(p.join(folder, filename));
    await file.writeAsBytes(response.bodyBytes);
  } catch (error) {
    print("Error while trying to download!");
    print(error);
  }
}

void main(List<String> args) async {
  print("Args: ${args.join(", ")}");

  if (args.isEmpty) {
    try {
      registerInstaller();
      print("Successfully installed OneClick installer.");
    } catch (exception) {
      print(
          "Error while trying to install... Try running the exe as Administrator.");
      print(exception);
    }
    print("This window closes automatically in 5 seconds");
    sleep(Duration(seconds: 5));
  } else if (args.length > 1) {
    var synthFolder = getSynthRidersFolder();
    if (synthFolder == null || !await checkIfSR(synthFolder)) {
      print(
          "Error: Couldn't find Synthriders folder! Please start Synthriders and then close it again.");
      sleep(Duration(seconds: 3));
      return;
    }

    synthFolder = synthFolder.substring(0, synthFolder.length - 1);

    // Options
    switch (args[0]) {
      case "--install":
        var _args = args[1].replaceFirst("synthriderz://", "");
        var commands = _args.split(";");
        taskClient = http.Client();
        for (var cmd in commands) {
          var opts = cmd.split("/");
          var id = int.tryParse(opts[0]);
          if (id != null) {
            // Old Noodle Link
            await downloadFile(
                urls["beatmaps"]!.replaceFirst("{{id}}", opts[0]),
                p.join(synthFolder, "CustomSongs"));
            continue;
          }
          switch (opts[0]) {
            case "beatmap":
              await downloadFile(
                  urls["beatmaps-hash"]!.replaceFirst("{{id}}", opts[1]),
                  p.join(synthFolder, "CustomSongs"));
              break;
            case "stage":
              if (opts.length < 2) continue;
              await downloadFile(
                  urls["stages"]!
                      .replaceFirst("{{id}}", opts[1])
                      .replaceFirst("{{id2}}", opts[2]),
                  p.join(synthFolder, "CustomStages"));
              break;
            case "avatar":
              await downloadFile(
                  urls["avatars"]!.replaceFirst("{{id}}", opts[1]),
                  p.join(synthFolder, "Avatars"));
              break;
            case "playlist":
              print("Not yet implemented!");
              break;
            case "mod":
              print("Not yet implemented!");
              break;
          }
        }
        taskClient.close();
        print("Done!");
        sleep(Duration(milliseconds: 750));
        return;
    }
  } else {
    print("Nothing to do!");
  }
}
