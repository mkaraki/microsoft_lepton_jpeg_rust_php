<?php
$jpg_path = 'original.jpg';
$lep_path = 'test.lep';
$jpg2_path = 'test2.jpg';

$jpg = file_get_contents($jpg_path);
if ($jpg === false) {
    die('Failed to read jpeg file.');
}

$lep = convert_jpeg_to_lepton($jpg);
if ($lep === null) {
    die('Failed to convert jpeg to lepton.');
}
file_put_contents($lep_path, $lep);

$jpg2 = convert_lepton_to_jpeg($lep);
if ($jpg2 === null) {
    die('Failed to convert lepton to jpeg.');
}
file_put_contents($jpg2_path, $jpg2);